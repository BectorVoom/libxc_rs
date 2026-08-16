//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 1069/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1069(t3768: f64, t3783: f64, t3727: f64, t489: f64, t1204: f64, t1234: f64, t1281: f64, t1285: f64, t1288: f64, t1291: f64, t3552: f64, t3666: f64, t3670: f64, t3746: f64, t3751: f64, t3755: f64, t3756: f64, t3760: f64, t3763: f64, t3767: f64, t3770: f64, t3774: f64, t3778: f64, t3782: f64, t460: f64, t490: f64) -> (f64, f64, f64) {
    let t3784 = t3768 * t3783;
    let t3787 = t489 * t3727;
    let t3790 = 0.65854491829355115987e0_f64 * t3552 * t490 - 0.13170898365871023197e1_f64 * t3666 * t1281 + 0.13170898365871023197e1_f64 * t3746 * t1288 + 0.13170898365871023197e1_f64 * t1204 * t1291 + 0.13170898365871023197e1_f64 * t3670 * t3751 - 0.13170898365871023197e1_f64 * t3755 * t3756 - 0.13170898365871023197e1_f64 * t1234 * t3760 - 0.65854491829355115987e0_f64 * t1234 * t3763 + 0.13170898365871023197e1_f64 * t3767 * t3770 + 0.13170898365871023197e1_f64 * t1285 * t3774 + 0.65854491829355115987e0_f64 * t1285 * t3778 - 0.65854491829355115987e0_f64 * t3782 * t3784 + 0.65854491829355115987e0_f64 * t460 * t3787;
    (t3784, t3787, t3790)
}
