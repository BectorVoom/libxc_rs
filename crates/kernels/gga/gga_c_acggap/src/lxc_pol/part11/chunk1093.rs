//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1093/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1093<F: Float>(t34937: F, t34941: F, t34946: F, t34949: F, t34953: F, t34958: F, t34962: F, t34965: F, t34969: F, t34973: F, t34977: F, t34980: F, t34984: F, t34986: F, t34987: F, t34991: F, t34994: F, t34996: F) -> F {
    let t34998 = F::new(0.62896184579208304136e-3) * t34937 - F::new(0.94344276868812456204e-2) * t34941 - t34946 + F::new(0.42874018118069736972e-3) * t34949 + F::new(0.21437009059034868486e-3) * t34953 + t34958 - t34962 - F::new(0.42874018118069736972e-3) * t34965 + F::new(0.15724046144802076034e-2) * t34969 - F::new(0.62896184579208304136e-3) * t34973 - F::new(0.10718504529517434243e-2) * t34977 + F::new(0.10718504529517434243e-2) * t34980 - F::new(0.64311027177104605458e-2) * t34984 - t34986 - t34987 + t34991 - F::new(0.12862205435420921092e-1) * t34994 - t34996 / F::new(48.0);
    t34998
}
