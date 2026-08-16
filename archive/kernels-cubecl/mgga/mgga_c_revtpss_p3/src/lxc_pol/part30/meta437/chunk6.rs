//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1683/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1683<F: Float>(t16781: F, t17169: F, t1287: F, t487: F, t3584: F, t5486: F, t16756: F, t5480: F, t1770: F, t3781: F, t1234: F, t12709: F, t12756: F, t1285: F, t1291: F, t16697: F, t16751: F, t16757: F, t16763: F, t16768: F, t16772: F, t16776: F, t3666: F, t3670: F, t3746: F, t3760: F, t3763: F, t3784: F, t5216: F, t5326: F, t5459: F, t5463: F, t5474: F, t5478: F, t5487: F) -> (F, F) {
    let t17170 = t16781 + t17169;
    let t17172 = t487 * t17170 * t1287;
    let t17175 = t5486 * t3584;
    let t17178 = t16756 * t5480;
    let t17183 = t1770 * t3781;
    let t17186 = F::cast_from(0.13170898365871023197e1_f64) * t12756 * t16697 - F::cast_from(0.13170898365871023197e1_f64) * t3666 * t5487 - F::cast_from(0.65854491829355115987e0_f64) * t1234 * t16751 + F::cast_from(0.13170898365871023197e1_f64) * t3746 * t5474 + F::cast_from(0.26341796731742046394e1_f64) * t5463 * t16757 + F::cast_from(0.13170898365871023197e1_f64) * t5216 * t1291 + F::cast_from(0.65854491829355115987e0_f64) * t1285 * t16763 - F::cast_from(0.65854491829355115987e0_f64) * t5326 * t3763 - F::cast_from(0.65854491829355115987e0_f64) * t1234 * t16768 + F::cast_from(0.26341796731742046394e1_f64) * t3670 * t16772 + F::cast_from(0.13170898365871023197e1_f64) * t3670 * t16776 - F::cast_from(0.13170898365871023197e1_f64) * t5326 * t3760 + F::cast_from(0.65854491829355115987e0_f64) * t1285 * t17172 - F::cast_from(0.65854491829355115987e0_f64) * t1234 * t17175 - F::cast_from(0.13170898365871023197e1_f64) * t5478 * t17178 - F::cast_from(0.13170898365871023197e1_f64) * t12709 * t5459 - F::cast_from(0.65854491829355115987e0_f64) * t17183 * t3784;
    (t17170, t17186)
}
