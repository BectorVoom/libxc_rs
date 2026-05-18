//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 960/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk960<F: Float>(t11695: F, t3225: F, t773: F, t826: F, t10264: F, t3212: F, t3724: F, t3209: F, t11659: F, t11664: F, t11666: F, t11671: F, t11676: F, t11680: F, t11685: F, t11689: F, t11692: F) -> (F, F) {
    let t11696 = t3225 * t11695;
    let t11698 = t826 * t773;
    let t11699 = t10264 * t11698;
    let t11701 = t3212 * t3724;
    let t11703 = t3209 * t3724;
    let t11705 = -F::new(0.24464544158376474785e-7) * t11659 + F::new(0.32109714207869123156e-6) * t11664 - F::new(0.11742981196020707897e-4) * t11666 - F::new(0.11742981196020707897e-4) * t11671 + F::new(0.17098714139140853038e-6) * t11676 + F::new(0.17098714139140853038e-6) * t11680 - F::new(0.73393632475129424356e-6) * t11685 - F::new(0.73393632475129424356e-6) * t11689 - F::new(0.20879020566524818641e-5) * t11692 + F::new(0.56995713797136176793e-7) * t11696 - F::new(0.27357942622625364861e-5) * t11699 + F::new(0.82073827867876094584e-5) * t11701 - F::new(0.3556532540941297432e-4) * t11703;
    (t11698, t11705)
}
