//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1017/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1017(t24562: f64, t24587: f64, t24622: f64, t24674: f64, t24722: f64, t24778: f64, t24815: f64, t24861: f64, t225: f64, t494: f64, t1210: f64, t1274: f64, t1775: f64, t17995: f64, t18059: f64, t1829: f64, t20697: f64, t20700: f64, t20753: f64, t21394: f64, t21621: f64, t24509: f64, t24515: f64, t24519: f64, t24525: f64, t24698: f64, t460: f64, t495: f64, t5220: f64, t5417: f64, t6574: f64, t6580: f64, t6745: f64) -> (f64, f64) {
    let t24864 = t24562 + t24587 + t24622 + t24674 + t24722 + t24778 + t24815 + t24861;
    let t24866 = t24864 * t225 * t494;
    let t24881 = 0.39512695097613069591e1_f64 * t17995 * t6574 + 0.39512695097613069591e1_f64 * t1274 * t24509 - 0.19756347548806534796e1_f64 * t20753 * t1829 + 0.19756347548806534796e1_f64 * t1210 * t24515 - 0.39512695097613069591e1_f64 * t1210 * t24519 - 0.19756347548806534796e1_f64 * t20700 * t1829 - 0.39512695097613069591e1_f64 * t1274 * t24525 - 0.19756347548806534796e1_f64 * t20697 * t1775 + 0.65854491829355115987e0_f64 * t460 * t24866 - 0.19756347548806534796e1_f64 * t5417 * t6745 + 0.39512695097613069591e1_f64 * t18059 * t6574 + 0.39512695097613069591e1_f64 * t5220 * t6580 - 0.39512695097613069591e1_f64 * t21394 * t1775 - 0.19756347548806534796e1_f64 * t21621 * t1775 + 0.65854491829355115987e0_f64 * t24698 * t495;
    (t24864, t24881)
}
