//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 692/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk692(t2355: f64, t3418: f64, t1016: f64, t3145: f64, t4349: f64, t921: f64, t1382: f64, t3207: f64, t12762: f64, t1445: f64, t597: f64, t12766: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12854 = t2355 * t3418;
    let t12855 = 2.0_f64 * t12854;
    let t12856 = t1016 * t3145;
    let t12858 = 6.0_f64 * t4349 * t12856;
    let t12859 = t3418 * t921;
    let t12860 = t1382 * t12859;
    let t12861 = 4.0_f64 * t12860;
    let t12862 = t1016 * t3207;
    let t12864 = 2.0_f64 * t1382 * t12862;
    let t12865 = t1445 * t12762;
    let t12866 = t597 * t12865;
    let t12868 = t1445 * t12766;
    (t12855, t12856, t12858, t12859, t12861, t12862, t12864, t12865, t12866, t12868)
}
