//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 933/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk933<F: Float>(t1897: F, t1901: F, t42980: F, t42981: F, t42982: F, t42983: F, t42984: F, t42986: F, t42989: F, t42992: F, t42993: F, t42998: F, t42999: F, t43003: F, t43006: F, t43010: F, t43014: F, t43017: F, t43019: F, t43023: F) -> F {
    let t43024 = -t42980 - t42981 + t42982 - t42983 + t42984 + t42986 + t42989 + t42992 + F::new(0.76905262301422242837e-2) * t1897 * t1901 * t42993 - t42998 + F::new(0.41016139894091862845e-1) * t42999 + F::new(0.30762104920568897134e-1) * t43003 + t43006 - F::new(0.34180116578409885704e-2) * t43010 - t43014 - t43017 + t43019 - t43023;
    t43024
}
