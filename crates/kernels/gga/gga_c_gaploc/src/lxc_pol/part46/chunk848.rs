//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 848/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk848<F: Float>(t10318: F, t1397: F, t9287: F, t2487: F, t2754: F, t9438: F, t9448: F, t204: F, t2476: F, t41810: F, t1445: F, t1562: F, t2854: F, t9127: F) -> (F, F, F, F) {
    let t41914 = t1397 * t10318 * t9287;
    let t41915 = F::new(0.29792074959875355558e-1) * t41914;
    let t41918 = t2487 * t9438 * t9448 * t2754;
    let t41919 = F::new(0.15976219147466979032e-1) * t41918;
    let t41922 = F::new(0.46011511144704899612e1) * t2476 * t204 * t41810;
    let t41927 = F::new(0.69017266717057349418e1) * t1562 * t1445 * t2854 * t9127;
    (t41915, t41919, t41922, t41927)
}
