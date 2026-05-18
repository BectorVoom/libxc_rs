//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 891/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk891<F: Float>(t7064: F, t7069: F, t8878: F, t13212: F, t7129: F, t40693: F, t40696: F, t40699: F, t40612: F, t40614: F, t40620: F, t40622: F, t40627: F, t40630: F, t40632: F, t40634: F, t471: F) -> (F, F, F, F, F, F) {
    let t43042 = t7064 * t8878 * t7069;
    let t43043 = F::new(0.1922631557535556071e-2) * t43042;
    let t43049 = F::new(0.23071578690426672851e-1) * t7129 * t13212;
    let t43053 = F::new(0.64087718584518535698e-3) * t40693;
    let t43054 = F::new(0.64087718584518535698e-3) * t40696;
    let t43055 = F::new(0.64087718584518535698e-3) * t40699;
    let t43069 = (F::new(21.0) / F::new(512.0) * t40612 + F::new(357.0) / F::new(16384.0) * t40614 - F::new(189.0) / F::new(262144.0) * t40620 + F::new(189.0) / F::new(0.16777216e8) * t40622 - F::new(63.0) / F::new(0.16777216e8) * t40627 + F::new(63.0) / F::new(262144.0) * t40630 - F::new(119.0) / F::new(16384.0) * t40632 - F::new(7.0) / F::new(512.0) * t40634) * t471;
    (t43043, t43049, t43053, t43054, t43055, t43069)
}
