//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 623/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk623<F: Float>(t487: F, t6628: F, t3769: F, t1287: F, t1794: F, t1811: F, t6622: F, t3783: F, t489: F, t6695: F, t1234: F, t1285: F, t1770: F, t1818: F, t1822: F, t1825: F, t3670: F, t3755: F, t3767: F, t3782: F, t460: F, t490: F, t5326: F, t5436: F, t6564: F, t6714: F, t6717: F, t6720: F, t6723: F) -> (F, F, F, F, F, F) {
    let t6726 = t487 * t6628;
    let t6727 = t6726 * t3769;
    let t6731 = t1811 * t1794 * t1287;
    let t6735 = t487 * t6622 * t1287;
    let t6738 = t6726 * t3783;
    let t6741 = t489 * t6695;
    let t6744 = F::new(0.65854491829355115987e0) * t6564 * t490 - F::new(0.13170898365871023197e1) * t5326 * t1818 + F::new(0.13170898365871023197e1) * t5436 * t1822 + F::new(0.13170898365871023197e1) * t1770 * t1825 + F::new(0.13170898365871023197e1) * t3670 * t6714 - F::new(0.13170898365871023197e1) * t3755 * t6717 - F::new(0.13170898365871023197e1) * t1234 * t6720 - F::new(0.65854491829355115987e0) * t1234 * t6723 + F::new(0.13170898365871023197e1) * t3767 * t6727 + F::new(0.13170898365871023197e1) * t1285 * t6731 + F::new(0.65854491829355115987e0) * t1285 * t6735 - F::new(0.65854491829355115987e0) * t3782 * t6738 + F::new(0.65854491829355115987e0) * t460 * t6741;
    (t6727, t6731, t6735, t6738, t6741, t6744)
}
