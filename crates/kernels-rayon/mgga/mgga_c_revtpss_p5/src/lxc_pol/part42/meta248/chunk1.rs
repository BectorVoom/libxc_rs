//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 940/1505 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk940(t1277: f64, t6744: f64, t1210: f64, t1274: f64, t1770: f64, t1775: f64, t1813: f64, t1829: f64, t3567: f64, t460: f64, t495: f64, t5220: f64, t5225: f64, t5251: f64, t5417: f64, t6564: f64, t6574: f64, t6580: f64, t6588: f64, t6697: f64, t6703: f64) -> (f64, f64) {
    let t6745 = t1277 * t6744;
    let t6748 = 0.65854491829355115987e0_f64 * t6564 * t495 - 0.13170898365871023197e1_f64 * t5220 * t1775 + 0.13170898365871023197e1_f64 * t1770 * t1813 - 0.13170898365871023197e1_f64 * t5225 * t1829 + 0.13170898365871023197e1_f64 * t3567 * t6574 - 0.13170898365871023197e1_f64 * t5251 * t1775 + 0.13170898365871023197e1_f64 * t1210 * t6580 - 0.65854491829355115987e0_f64 * t1210 * t6588 + 0.65854491829355115987e0_f64 * t460 * t6697 - 0.13170898365871023197e1_f64 * t5417 * t1829 + 0.13170898365871023197e1_f64 * t1274 * t6703 - 0.65854491829355115987e0_f64 * t1274 * t6745;
    (t6745, t6748)
}
