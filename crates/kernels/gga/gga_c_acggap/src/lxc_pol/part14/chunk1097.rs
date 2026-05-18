//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1097/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1097<F: Float>(t1181: F, t604: F, t6192: F, t7426: F, t30330: F, t30334: F, t30340: F, t30343: F, t30347: F, t34309: F, t34312: F, t34336: F, t34341: F, t34348: F, t37021: F, t37022: F, t39182: F, t39186: F, t39189: F, t39192: F, t39194: F) -> F {
    let t39203 = t7426 * t1181 * t604 * t6192;
    let t39205 = -F::new(0.10718504529517434243e-2) * t39182 - F::new(0.10718504529517434243e-2) * t39186 - F::new(0.7145669686344956162e-3) * t39189 + F::new(0.80031500487063509015e-2) * t34309 + t34312 + t37021 + t37022 + F::new(0.17149607247227894789e-1) * t39192 - F::new(0.68598428988911579156e-2) * t39194 - F::new(0.10718504529517434243e-2) * t30330 - F::new(0.42874018118069736972e-3) * t30334 + t30340 + F::new(0.62896184579208304136e-3) * t34336 + F::new(0.53592522647587171215e-3) * t30343 + F::new(0.21437009059034868486e-3) * t30347 + t34341 + F::new(0.31448092289604152068e-3) * t39203 - t34348;
    t39205
}
