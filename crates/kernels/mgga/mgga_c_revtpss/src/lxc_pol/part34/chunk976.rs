//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 976/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk976<F: Float>(t24562: F, t24587: F, t24622: F, t24674: F, t24722: F, t24778: F, t24815: F, t24861: F, t225: F, t494: F, t1210: F, t1274: F, t1775: F, t17995: F, t18059: F, t1829: F, t20697: F, t20700: F, t20753: F, t21394: F, t21621: F, t24509: F, t24515: F, t24519: F, t24525: F, t24698: F, t460: F, t495: F, t5220: F, t5417: F, t6574: F, t6580: F, t6745: F) -> (F, F) {
    let t24864 = t24562 + t24587 + t24622 + t24674 + t24722 + t24778 + t24815 + t24861;
    let t24866 = t24864 * t225 * t494;
    let t24881 = 0.39512695097613069591e1 * t17995 * t6574 + 0.39512695097613069591e1 * t1274 * t24509 - 0.19756347548806534796e1 * t20753 * t1829 + 0.19756347548806534796e1 * t1210 * t24515 - 0.39512695097613069591e1 * t1210 * t24519 - 0.19756347548806534796e1 * t20700 * t1829 - 0.39512695097613069591e1 * t1274 * t24525 - 0.19756347548806534796e1 * t20697 * t1775 + 0.65854491829355115987e0 * t460 * t24866 - 0.19756347548806534796e1 * t5417 * t6745 + 0.39512695097613069591e1 * t18059 * t6574 + 0.39512695097613069591e1 * t5220 * t6580 - 0.39512695097613069591e1 * t21394 * t1775 - 0.19756347548806534796e1 * t21621 * t1775 + 0.65854491829355115987e0 * t24698 * t495;
    (t24864, t24881)
}
