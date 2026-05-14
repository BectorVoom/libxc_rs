//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 600/1170 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk600<F: Float>(t3768: F, t3783: F, t3727: F, t489: F, t1204: F, t1234: F, t1281: F, t1285: F, t1288: F, t1291: F, t3552: F, t3666: F, t3670: F, t3746: F, t3751: F, t3755: F, t3756: F, t3760: F, t3763: F, t3767: F, t3770: F, t3774: F, t3778: F, t3782: F, t460: F, t490: F) -> (F, F, F) {
    let t3784 = t3768 * t3783;
    let t3787 = t489 * t3727;
    let t3790 = 0.65854491829355115987e0 * t3552 * t490 - 0.13170898365871023197e1 * t3666 * t1281 + 0.13170898365871023197e1 * t3746 * t1288 + 0.13170898365871023197e1 * t1204 * t1291 + 0.13170898365871023197e1 * t3670 * t3751 - 0.13170898365871023197e1 * t3755 * t3756 - 0.13170898365871023197e1 * t1234 * t3760 - 0.65854491829355115987e0 * t1234 * t3763 + 0.13170898365871023197e1 * t3767 * t3770 + 0.13170898365871023197e1 * t1285 * t3774 + 0.65854491829355115987e0 * t1285 * t3778 - 0.65854491829355115987e0 * t3782 * t3784 + 0.65854491829355115987e0 * t460 * t3787;
    (t3784, t3787, t3790)
}
