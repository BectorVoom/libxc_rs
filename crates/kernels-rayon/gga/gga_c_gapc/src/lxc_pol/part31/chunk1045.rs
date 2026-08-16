//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1045/1447 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1045(t11205: f64, t11212: f64, t11218: f64, t11220: f64, t11225: f64, t11229: f64, t11231: f64, t11240: f64, t11244: f64, t11246: f64, t11250: f64, t11255: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12016 = 0.2530696388073708253e-5_f64 * t11205;
    let t12017 = 0.18103800586153667463e-6_f64 * t11212;
    let t12018 = 0.23761238269326688546e-5_f64 * t11218;
    let t12019 = 0.86898242813537603825e-4_f64 * t11220;
    let t12020 = 0.86898242813537603825e-4_f64 * t11225;
    let t12021 = 0.2530696388073708253e-5_f64 * t11229;
    let t12022 = 0.3475929712541504153e-3_f64 * t11231;
    let t12025 = 0.1545050757224698596e-4_f64 * t11240;
    let t12026 = 0.84356546269123608433e-6_f64 * t11244;
    let t12027 = 0.52638484871933131665e-3_f64 * t11246;
    let t12028 = 0.32188557442181220751e-6_f64 * t11250;
    let t12030 = 0.86898242813537603825e-4_f64 * t11255;
    (t12016, t12017, t12018, t12019, t12020, t12021, t12022, t12025, t12026, t12027, t12028, t12030)
}
