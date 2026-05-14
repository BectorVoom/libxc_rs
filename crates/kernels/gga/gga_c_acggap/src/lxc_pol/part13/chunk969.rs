//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 969/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk969<F: Float>(t30786: F, t30790: F, t1992: F, t5606: F, t7585: F, t7586: F, t1181: F, t4257: F, t604: F, t8463: F, t4791: F, t570: F, t34937: F, t34941: F, t34946: F, t34949: F, t34953: F, t34958: F, t34962: F, t34965: F, t34969: F, t34973: F, t34977: F, t34980: F, t34984: F) -> (F,) {
    let t34986 = 0.21437009059034868486e-3 * t30786;
    let t34987 = 0.28582678745379824648e-3 * t30790;
    let t34990 = t7585 * t7586 * t1992 * t5606;
    let t34991 = 0.28582678745379824648e-3 * t34990;
    let t34994 = t8463 * t1181 * t604 * t4257;
    let t34996 = t570 * t4791;
    let t34998 = 0.62896184579208304136e-3 * t34937 - 0.94344276868812456204e-2 * t34941 - t34946 + 0.42874018118069736972e-3 * t34949 + 0.21437009059034868486e-3 * t34953 + t34958 - t34962 - 0.42874018118069736972e-3 * t34965 + 0.15724046144802076034e-2 * t34969 - 0.62896184579208304136e-3 * t34973 - 0.10718504529517434243e-2 * t34977 + 0.10718504529517434243e-2 * t34980 - 0.64311027177104605458e-2 * t34984 - t34986 - t34987 + t34991 - 0.12862205435420921092e-1 * t34994 - t34996 / 48.0;
    (t34998,)
}
