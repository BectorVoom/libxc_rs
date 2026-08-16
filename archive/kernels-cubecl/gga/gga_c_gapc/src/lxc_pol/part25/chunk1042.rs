//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 1042/1444 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk1042<F: Float>(t11205: F, t11212: F, t11218: F, t11220: F, t11225: F, t11229: F, t11231: F, t11240: F, t11244: F, t11246: F, t11250: F, t11255: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t12016 = F::cast_from(0.2530696388073708253e-5_f64) * t11205;
    let t12017 = F::cast_from(0.18103800586153667463e-6_f64) * t11212;
    let t12018 = F::cast_from(0.23761238269326688546e-5_f64) * t11218;
    let t12019 = F::cast_from(0.86898242813537603825e-4_f64) * t11220;
    let t12020 = F::cast_from(0.86898242813537603825e-4_f64) * t11225;
    let t12021 = F::cast_from(0.2530696388073708253e-5_f64) * t11229;
    let t12022 = F::cast_from(0.3475929712541504153e-3_f64) * t11231;
    let t12025 = F::cast_from(0.1545050757224698596e-4_f64) * t11240;
    let t12026 = F::cast_from(0.84356546269123608433e-6_f64) * t11244;
    let t12027 = F::cast_from(0.52638484871933131665e-3_f64) * t11246;
    let t12028 = F::cast_from(0.32188557442181220751e-6_f64) * t11250;
    let t12030 = F::cast_from(0.86898242813537603825e-4_f64) * t11255;
    (t12016, t12017, t12018, t12019, t12020, t12021, t12022, t12025, t12026, t12027, t12028, t12030)
}
