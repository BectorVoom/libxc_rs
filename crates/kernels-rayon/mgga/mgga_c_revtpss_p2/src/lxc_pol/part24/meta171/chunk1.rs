//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 843/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk843(t489: f64, t6695: f64, t1234: f64, t1285: f64, t1770: f64, t1818: f64, t1822: f64, t1825: f64, t3670: f64, t3755: f64, t3767: f64, t3782: f64, t460: f64, t490: f64, t5326: f64, t5436: f64, t6564: f64, t6714: f64, t6717: f64, t6720: f64, t6723: f64, t6727: f64, t6731: f64, t6735: f64, t6738: f64) -> (f64, f64) {
    let t6741 = t489 * t6695;
    let t6744 = 0.65854491829355115987e0_f64 * t6564 * t490 - 0.13170898365871023197e1_f64 * t5326 * t1818 + 0.13170898365871023197e1_f64 * t5436 * t1822 + 0.13170898365871023197e1_f64 * t1770 * t1825 + 0.13170898365871023197e1_f64 * t3670 * t6714 - 0.13170898365871023197e1_f64 * t3755 * t6717 - 0.13170898365871023197e1_f64 * t1234 * t6720 - 0.65854491829355115987e0_f64 * t1234 * t6723 + 0.13170898365871023197e1_f64 * t3767 * t6727 + 0.13170898365871023197e1_f64 * t1285 * t6731 + 0.65854491829355115987e0_f64 * t1285 * t6735 - 0.65854491829355115987e0_f64 * t3782 * t6738 + 0.65854491829355115987e0_f64 * t460 * t6741;
    (t6741, t6744)
}
