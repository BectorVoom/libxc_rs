//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 935/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk935<F: Float>(t1445: F, t2087: F, t43240: F, t13161: F, t5782: F, t13149: F, t2464: F, t825: F, t24968: F, t9958: F, t43598: F, t7572: F, t7573: F) -> (F, F, F, F, F) {
    let t44038 = F::new(0.62115540045351614476e2) * t2087 * t1445 * t43240;
    let t44040 = F::new(0.62115540045351614476e2) * t5782 * t13161;
    let t44045 = t825 * t2464 * t13149;
    let t44046 = F::new(0.63904876589867916128e-1) * t44045;
    let t44051 = F::new(0.42900587942220512003e1) * t24968 * t9958;
    let t44057 = F::new(0.62115540045351614476e2) * t7572 * t7573 * t43598;
    (t44038, t44040, t44046, t44051, t44057)
}
