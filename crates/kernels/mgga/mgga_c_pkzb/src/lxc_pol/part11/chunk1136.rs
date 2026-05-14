//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1136/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1136<F: Float>(t2860: F, t9533: F, t9356: F, t10829: F, t1954: F, t723: F, t730: F, t10964: F, t1987: F, t10955: F, t2874: F, t10952: F, t30385: F, t30387: F, t30502: F, t30620: F, t30622: F, t30624: F, t30626: F, t30628: F, t30637: F, t30704: F, t30706: F, t30708: F, t30710: F, t30714: F) -> (F, F, F, F, F, F, F) {
    let t30716 = 0.30762056574649219972e4 * t2860 * t9533;
    let t30718 = 0.35089341735807877242e1 * t2860 * t9356;
    let t30722 = 0.11696447245269292414e1 * t730 * t1954 * t10829 * t723;
    let t30724 = 0.10389515463408878255e3 * t1987 * t10964;
    let t30727 = 0.6233709278045326953e3 * t730 * t10955 * t2874;
    let t30729 = 0.51947577317044391277e2 * t1987 * t10952;
    let t30730 = t30385 + t30387 + t30704 + t30502 - t30706 + t30708 + t30710 + t30714 - t30716 + t30718 + t30722 + t30724 - t30727 - t30729 + t30620 + t30622 + t30624 - t30626 - t30628 - t30637;
    (t30716, t30718, t30722, t30724, t30727, t30729, t30730)
}
