//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 981/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk981<F: Float>(t11082: F, t203: F, t184: F, t221: F, t10801: F, t10804: F, t10807: F, t10810: F, t10813: F, t10816: F, t10819: F, t10823: F, t10825: F, t10827: F, t10830: F, t4940: F, t4941: F, t7374: F, t7376: F, t7378: F, t7549: F) -> (F, F) {
    let t11083 = t203 * t11082;
    let t11084 = t11083 * t184;
    let t11086 = F::new(2.0) / F::new(15.0) * t11084 * t221;
    let t11102 = t4940 + F::new(0.83962962962962962963e-3) * t4941 + F::new(0.16792592592592592593e-2) * t7374 - F::new(0.83962962962962962967e-3) * t7378 + t7549 - F::new(0.2518888888888888889e-2) * t7376 - F::new(0.41981481481481481483e-3) * t10823 + F::new(0.20990740740740740742e-2) * t10801 - F::new(0.75566666666666666669e-2) * t10804 + F::new(0.5037777777777777778e-2) * t10807 + F::new(0.12594444444444444445e-2) * t10825 + F::new(0.11335e-1) * t10810 - F::new(0.15113333333333333334e-1) * t10813 - F::new(0.62972222222222222223e-3) * t10827 + F::new(0.12594444444444444445e-2) * t10816 - F::new(0.37783333333333333334e-2) * t10819 + F::new(0.18891666666666666667e-2) * t10830;
    (t11086, t11102)
}
