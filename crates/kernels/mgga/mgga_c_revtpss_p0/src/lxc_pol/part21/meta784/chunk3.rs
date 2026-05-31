//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2824/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2824<F: Float>(t10627: F, t10818: F, t11075: F, t14375: F, t14749: F, t14767: F, t1544: F, t1583: F, t198: F, t2403: F, t2404: F, t39419: F, t39422: F, t39429: F, t39432: F, t39483: F, t39520: F, t39528: F, t39531: F, t39534: F, t39537: F, t39540: F, t39773: F, t40067: F, t40072: F, t40099: F, t40103: F, t40115: F, t41137: F, t4343: F, t4541: F, t4546: F, t4556: F, t49865: F, t49867: F, t49868: F, t49869: F, t49870: F, t49903: F, t49912: F, t49913: F, t49921: F, t49925: F, t49927: F, t49930: F, t49941: F, t49944: F, t49945: F, t49972: F, t49988: F, t50040: F, t50045: F, t50046: F, t50048: F, t50051: F, t50055: F, t50056: F, t50078: F, t50102: F, t50861: F, t50864: F, t50866: F, t50869: F, t50871: F, t50872: F, t51769: F, t51775: F, t51786: F, t51802: F, t51810: F, t892: F) -> F {
    let t51814 = -t39483 + t39531 + t39773 + t39520 - t40115 + t39537 - t39540 - t39528 - t40072 + t51802 + t40103 + t51769 + F::cast_from(6.0_f64) * t198 * t10627 * t1583 * t892 + t51810 - t39429 + t50866 - t39419 - t39422 + t50869 + t50871 - t50872 + t51786 + t39534 + t40067 - t39432 + t50861 + F::cast_from(6.0_f64) * t2403 * t41137 * t1544 + F::cast_from(18.0_f64) * t198 * t14375 * t4343 - F::cast_from(18.0_f64) * t4541 * t4556 * t51775 + F::cast_from(9.0_f64) * t2403 * t11075 * t4343 + F::cast_from(36.0_f64) * t4541 * t2404 * t14749 + F::cast_from(18.0_f64) * t4541 * t2404 * t14767 + F::cast_from(18.0_f64) * t4541 * t4546 * t10818 + t50864 + t40099 + t50102 + t50078 + t50056 + t50055 + t50051 + t50045 - t50046 + t50048 + t50040 + t49988 + t49972 + t49944 - t49945 + t49941 + t49930 + t49925 - t49927 + t49921 + t49912 + t49913 + t49903 - t49869 + t49870 + t49865 - t49867 - t49868;
    t51814
}
