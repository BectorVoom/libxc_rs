//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1252/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1252(t10964: f64, t1987: f64, t10955: f64, t2874: f64, t730: f64, t10952: f64, t30385: f64, t30387: f64, t30502: f64, t30620: f64, t30622: f64, t30624: f64, t30626: f64, t30628: f64, t30637: f64, t30704: f64, t30706: f64, t30708: f64, t30710: f64, t30714: f64, t30716: f64, t30718: f64, t30722: f64) -> (f64, f64, f64, f64) {
    let t30724 = 0.10389515463408878255e3_f64 * t1987 * t10964;
    let t30727 = 0.6233709278045326953e3_f64 * t730 * t10955 * t2874;
    let t30729 = 0.51947577317044391277e2_f64 * t1987 * t10952;
    let t30730 = t30385 + t30387 + t30704 + t30502 - t30706 + t30708 + t30710 + t30714 - t30716 + t30718 + t30722 + t30724 - t30727 - t30729 + t30620 + t30622 + t30624 - t30626 - t30628 - t30637;
    (t30724, t30727, t30729, t30730)
}
