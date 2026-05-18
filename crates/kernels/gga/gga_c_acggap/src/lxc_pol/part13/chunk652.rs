//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 652/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk652<F: Float>(t1524: F, t174: F, t301: F, t960: F, t1586: F, t372: F, t1165: F, t3176: F, t4417: F, t1150: F, t1173: F, t1180: F, t335: F, t367: F, t3671: F, t3673: F, t3677: F, t3679: F, t3686: F, t3694: F, t3699: F, t3702: F, t3703: F, t3733: F, t3741: F, t5157: F, t5161: F, t5165: F, t5169: F, t5171: F, t5175: F) -> (F, F, F, F) {
    let t5182 = t174 * t1524;
    let t5183 = t5182 * t301;
    let t5184 = t960 * t5183;
    let t5187 = t1586 * t372;
    let t5188 = t960 * t5187;
    let t5192 = t1165 * t4417 * t3176;
    let t5197 = -F::new(0.34299214494455789578e-2) * t1173 * t5157 - t335 * t5161 / F::new(24.0) - t1150 * t5165 / F::new(16.0) - t5169 - t367 * t5171 / F::new(16.0) - t5175 - F::new(0.45351183609335988442e-1) * t3671 + F::new(0.22675591804667994222e-1) * t3673 - F::new(0.22675591804667994222e-1) * t3677 + F::new(0.16006300097412701803e-1) * t3679 - F::new(0.42874018118069736972e-3) * t3686 - t3694 - t3699 - t3702 + F::new(0.12862205435420921092e-2) * t3703 + t335 * t5184 / F::new(24.0) + t367 * t5188 / F::new(24.0) - F::new(0.25724410870841842184e-2) * t1180 * t5192 - F::new(0.42874018118069736972e-3) * t3733 + F::new(0.40015750243531754508e-2) * t3741;
    (t5183, t5187, t5192, t5197)
}
