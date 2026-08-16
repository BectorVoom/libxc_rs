//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2824/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2824(t10627: f64, t10818: f64, t11075: f64, t14375: f64, t14749: f64, t14767: f64, t1544: f64, t1583: f64, t198: f64, t2403: f64, t2404: f64, t39419: f64, t39422: f64, t39429: f64, t39432: f64, t39483: f64, t39520: f64, t39528: f64, t39531: f64, t39534: f64, t39537: f64, t39540: f64, t39773: f64, t40067: f64, t40072: f64, t40099: f64, t40103: f64, t40115: f64, t41137: f64, t4343: f64, t4541: f64, t4546: f64, t4556: f64, t49865: f64, t49867: f64, t49868: f64, t49869: f64, t49870: f64, t49903: f64, t49912: f64, t49913: f64, t49921: f64, t49925: f64, t49927: f64, t49930: f64, t49941: f64, t49944: f64, t49945: f64, t49972: f64, t49988: f64, t50040: f64, t50045: f64, t50046: f64, t50048: f64, t50051: f64, t50055: f64, t50056: f64, t50078: f64, t50102: f64, t50861: f64, t50864: f64, t50866: f64, t50869: f64, t50871: f64, t50872: f64, t51769: f64, t51775: f64, t51786: f64, t51802: f64, t51810: f64, t892: f64) -> f64 {
    let t51814 = -t39483 + t39531 + t39773 + t39520 - t40115 + t39537 - t39540 - t39528 - t40072 + t51802 + t40103 + t51769 + 6.0_f64 * t198 * t10627 * t1583 * t892 + t51810 - t39429 + t50866 - t39419 - t39422 + t50869 + t50871 - t50872 + t51786 + t39534 + t40067 - t39432 + t50861 + 6.0_f64 * t2403 * t41137 * t1544 + 18.0_f64 * t198 * t14375 * t4343 - 18.0_f64 * t4541 * t4556 * t51775 + 9.0_f64 * t2403 * t11075 * t4343 + 36.0_f64 * t4541 * t2404 * t14749 + 18.0_f64 * t4541 * t2404 * t14767 + 18.0_f64 * t4541 * t4546 * t10818 + t50864 + t40099 + t50102 + t50078 + t50056 + t50055 + t50051 + t50045 - t50046 + t50048 + t50040 + t49988 + t49972 + t49944 - t49945 + t49941 + t49930 + t49925 - t49927 + t49921 + t49912 + t49913 + t49903 - t49869 + t49870 + t49865 - t49867 - t49868;
    t51814
}
