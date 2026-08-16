//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1376/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1376(t13881: f64, t4546: f64, t1597: f64, t3008: f64, t343: f64, t2960: f64, t4506: f64, t10263: f64, t13850: f64, t13852: f64, t13855: f64, t13858: f64, t13862: f64, t13865: f64, t13868: f64, t13871: f64, t13874: f64, t13877: f64, t1593: f64, t2986: f64, t973: f64) -> f64 {
    let t13882 = t4546 * t13881;
    let t13886 = t1597 * t3008 * t343;
    let t13887 = t4546 * t13886;
    let t13893 = 0.49382716049382716048e-3_f64 * t2960 * t4506;
    let t13894 = -t13850 - 0.55555555555555555554e-3_f64 * t2986 * t13852 - 0.27777777777777777777e-3_f64 * t2986 * t13855 - 0.27777777777777777777e-3_f64 * t2986 * t13858 - 0.55555555555555555554e-3_f64 * t2986 * t13862 - 0.11111111111111111111e-2_f64 * t2986 * t13865 - 0.55555555555555555554e-3_f64 * t2986 * t13868 - 0.27777777777777777777e-3_f64 * t2986 * t13871 + 0.16666666666666666666e-2_f64 * t2986 * t13874 - 0.22222222222222222221e-2_f64 * t2986 * t13877 - 0.83333333333333333332e-3_f64 * t973 * t13882 - 0.83333333333333333332e-3_f64 * t973 * t13887 + 0.27160493827160493826e-2_f64 * t10263 * t1593 - t13893;
    t13894
}
