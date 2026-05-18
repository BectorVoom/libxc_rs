//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1295/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1295<F: Float>(t4580: F, t95547: F, t95830: F, t13156: F, t2842: F, t7718: F, t26748: F, t26808: F, t27773: F, t27832: F, t27950: F, t2850: F, t2894: F, t2911: F, t44657: F, t7703: F, t93592: F, t95816: F, t95817: F, t95820: F, t95827: F, t95828: F) -> (F, F, F) {
    let t95832 = t95830 * t4580 * t95547;
    let t95844 = t2842 * t7718 * t13156;
    let t95846 = -t95816 - F::new(0.73697530864197530861e-3) * t95817 - F::new(0.33163888888888888888e-2) * t95820 + F::new(0.41703125000000000001e-2) * t7703 * t44657 * t27773 * t2911 + t95827 - F::new(0.55273148148148148147e-2) * t95828 - F::new(0.92673611111111111112e-3) * t93592 * t95832 - F::new(0.46336805555555555556e-3) * t7703 * t2894 * t27773 * t2850 + F::new(0.46336805555555555556e-3) * t27832 * t26808 - F::new(0.61782407407407407408e-3) * t26748 * t27950 - F::new(0.16581944444444444444e-1) * t95844;
    (t95832, t95844, t95846)
}
