//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1096/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1096<F: Float>(t24562: F, t24587: F, t24622: F, t24674: F, t24722: F, t24778: F, t24815: F, t24861: F, t225: F, t494: F, t1210: F, t1274: F, t1775: F, t17995: F, t18059: F, t1829: F, t20697: F, t20700: F, t20753: F, t21394: F, t21621: F, t24509: F, t24515: F, t24519: F, t24525: F, t24698: F, t460: F, t495: F, t5220: F, t5417: F, t6574: F, t6580: F, t6745: F) -> (F, F) {
    let t24864 = t24562 + t24587 + t24622 + t24674 + t24722 + t24778 + t24815 + t24861;
    let t24866 = t24864 * t225 * t494;
    let t24881 = F::cast_from(0.39512695097613069591e1_f64) * t17995 * t6574 + F::cast_from(0.39512695097613069591e1_f64) * t1274 * t24509 - F::cast_from(0.19756347548806534796e1_f64) * t20753 * t1829 + F::cast_from(0.19756347548806534796e1_f64) * t1210 * t24515 - F::cast_from(0.39512695097613069591e1_f64) * t1210 * t24519 - F::cast_from(0.19756347548806534796e1_f64) * t20700 * t1829 - F::cast_from(0.39512695097613069591e1_f64) * t1274 * t24525 - F::cast_from(0.19756347548806534796e1_f64) * t20697 * t1775 + F::cast_from(0.65854491829355115987e0_f64) * t460 * t24866 - F::cast_from(0.19756347548806534796e1_f64) * t5417 * t6745 + F::cast_from(0.39512695097613069591e1_f64) * t18059 * t6574 + F::cast_from(0.39512695097613069591e1_f64) * t5220 * t6580 - F::cast_from(0.39512695097613069591e1_f64) * t21394 * t1775 - F::cast_from(0.19756347548806534796e1_f64) * t21621 * t1775 + F::cast_from(0.65854491829355115987e0_f64) * t24698 * t495;
    (t24864, t24881)
}
