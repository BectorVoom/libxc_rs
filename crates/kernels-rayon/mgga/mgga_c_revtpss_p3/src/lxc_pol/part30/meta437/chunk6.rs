//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1683/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1683(t16781: f64, t17169: f64, t1287: f64, t487: f64, t3584: f64, t5486: f64, t16756: f64, t5480: f64, t1770: f64, t3781: f64, t1234: f64, t12709: f64, t12756: f64, t1285: f64, t1291: f64, t16697: f64, t16751: f64, t16757: f64, t16763: f64, t16768: f64, t16772: f64, t16776: f64, t3666: f64, t3670: f64, t3746: f64, t3760: f64, t3763: f64, t3784: f64, t5216: f64, t5326: f64, t5459: f64, t5463: f64, t5474: f64, t5478: f64, t5487: f64) -> (f64, f64) {
    let t17170 = t16781 + t17169;
    let t17172 = t487 * t17170 * t1287;
    let t17175 = t5486 * t3584;
    let t17178 = t16756 * t5480;
    let t17183 = t1770 * t3781;
    let t17186 = 0.13170898365871023197e1_f64 * t12756 * t16697 - 0.13170898365871023197e1_f64 * t3666 * t5487 - 0.65854491829355115987e0_f64 * t1234 * t16751 + 0.13170898365871023197e1_f64 * t3746 * t5474 + 0.26341796731742046394e1_f64 * t5463 * t16757 + 0.13170898365871023197e1_f64 * t5216 * t1291 + 0.65854491829355115987e0_f64 * t1285 * t16763 - 0.65854491829355115987e0_f64 * t5326 * t3763 - 0.65854491829355115987e0_f64 * t1234 * t16768 + 0.26341796731742046394e1_f64 * t3670 * t16772 + 0.13170898365871023197e1_f64 * t3670 * t16776 - 0.13170898365871023197e1_f64 * t5326 * t3760 + 0.65854491829355115987e0_f64 * t1285 * t17172 - 0.65854491829355115987e0_f64 * t1234 * t17175 - 0.13170898365871023197e1_f64 * t5478 * t17178 - 0.13170898365871023197e1_f64 * t12709 * t5459 - 0.65854491829355115987e0_f64 * t17183 * t3784;
    (t17170, t17186)
}
