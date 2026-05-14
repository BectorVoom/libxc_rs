//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 807/884 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk807<F: Float>(t43007: F, t7290: F, t1841: F, t7289: F, t2508: F, t3255: F, t8637: F, t2936: F, t9689: F, t13206: F, t7137: F, t3487: F, t734: F, t9636: F, t1897: F, t1901: F, t42980: F, t42981: F, t42982: F, t42983: F, t42984: F, t42986: F, t42989: F, t42992: F, t42993: F, t42998: F, t42999: F, t43003: F, t43006: F) -> (F, F) {
    let t43008 = t7290 * t43007;
    let t43010 = t1841 * t7289 * t43008;
    let t43014 = 0.23071578690426672851e-1 * t2508 * t8637 * t3255;
    let t43017 = 0.23071578690426672851e-1 * t2508 * t2936 * t9689;
    let t43019 = 0.20508069947045931423e-1 * t7137 * t13206;
    let t43023 = 0.85450291446024714263e-3 * t1841 * t9636 * t3487 * t734;
    let t43024 = -t42980 - t42981 + t42982 - t42983 + t42984 + t42986 + t42989 + t42992 + 0.76905262301422242837e-2 * t1897 * t1901 * t42993 - t42998 + 0.41016139894091862845e-1 * t42999 + 0.30762104920568897134e-1 * t43003 + t43006 - 0.34180116578409885704e-2 * t43010 - t43014 - t43017 + t43019 - t43023;
    (t43008, t43024)
}
