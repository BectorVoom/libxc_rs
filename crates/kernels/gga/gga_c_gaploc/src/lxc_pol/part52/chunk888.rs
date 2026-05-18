//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 888/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk888<F: Float>(t1445: F, t2087: F, t44995: F, t13648: F, t2194: F, t35558: F, t813: F, t935: F, t13621: F, t5782: F, t36390: F, t787: F, t9824: F) -> (F, F, F, F, F) {
    let t45892 = F::new(0.62115540045351614476e2) * t2087 * t1445 * t44995;
    let t45894 = F::new(0.46011511144704899612e1) * t2194 * t13648;
    let t45898 = F::new(0.46011511144704899612e1) * t813 * t1445 * t35558 * t935;
    let t45900 = F::new(0.69017266717057349418e1) * t5782 * t13621;
    let t45902 = t787 * t36390 * t9824;
    (t45892, t45894, t45898, t45900, t45902)
}
