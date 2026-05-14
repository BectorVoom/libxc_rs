//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 976/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk976<F: Float>(t13879: F, t1897: F, t702: F, t13941: F, t2508: F, t779: F, t42980: F, t42981: F, t42982: F, t42983: F, t42984: F, t42986: F, t42989: F, t42992: F, t42998: F, t42999: F, t43003: F, t43006: F, t43010: F, t43014: F, t43017: F, t43019: F, t43023: F, t43028: F, t43032: F, t43035: F) -> (F, F) {
    let t47616 = 0.76905262301422242837e-2 * t1897 * t13879 * t702;
    let t47619 = 0.76905262301422242837e-2 * t2508 * t779 * t13941;
    let t47620 = -t42980 - t42981 + t42982 - t42983 + t42984 + t42986 + t42989 + t42992 - t47616 + t47619 - t42998;
    let t47625 = 0.20508069947045931423e-1 * t42999 + 0.15381052460284448567e-1 * t43003 + t43006 - 0.17090058289204942852e-2 * t43010 - t43014 - t43017 + t43019 - t43023 + t43028 + t43032 - 0.85450291446024714263e-3 * t43035;
    (t47620, t47625)
}
