//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 696/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk696<F: Float>(t11013: F, t1866: F, t3281: F, t1755: F, t925: F, t1564: F, t446: F, t1882: F, t2989: F, t2994: F, t2985: F, t7775: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t11014 = t1866 * t11013;
    let t11015 = t3281 * t11014;
    let t11017 = t925 * t1755;
    let t11018 = t1564 * t11017;
    let t11019 = t446 * t11018;
    let t11021 = t1882 * t2989;
    let t11022 = t11021 / F::cast_from(27.0_f64);
    let t11023 = t1882 * t2994;
    let t11024 = F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t11023;
    let t11025 = t1882 * t2985;
    let t11026 = F::cast_from(2.0_f64) / F::cast_from(81.0_f64) * t11025;
    let t11027 = F::cast_from(4.0_f64) / F::cast_from(81.0_f64) * t7775;
    (t11015, t11017, t11019, t11021, t11022, t11023, t11024, t11025, t11026, t11027)
}
