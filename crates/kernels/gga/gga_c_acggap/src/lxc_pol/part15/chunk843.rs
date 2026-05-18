//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 843/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk843<F: Float>(t8233: F, t8235: F, t8240: F, t8835: F, t8841: F, t8847: F, t8851: F, t8860: F, t8862: F, t8876: F, t8879: F, t8882: F, t9682: F, t9688: F, t9692: F, t9694: F, t9696: F, t9698: F, t9702: F, t9706: F) -> F {
    let t9940 = -F::new(0.10718504529517434243e-2) * t9682 - t8233 + F::new(0.80031500487063509014e-2) * t8835 + F::new(0.34299214494455789578e-2) * t8841 - F::new(0.34299214494455789578e-2) * t8847 + F::new(0.42874018118069736972e-3) * t8851 + F::new(0.28582678745379824648e-3) * t8860 + F::new(0.21437009059034868486e-3) * t9688 - F::new(0.62896184579208304138e-3) * t9692 - F::new(0.34299214494455789578e-2) * t9694 + F::new(0.17149607247227894789e-2) * t9696 - F::new(0.17149607247227894789e-2) * t9698 + F::new(0.22921875e-1) * t9702 + F::new(0.1528125e-1) * t9706 + F::new(0.75475421495049964965e-2) * t8862 - t8876 / F::new(16.0) - t8879 / F::new(48.0) + t8235 - t8240 - F::new(0.1120625e0) * t8882;
    t9940
}
