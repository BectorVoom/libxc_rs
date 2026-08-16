//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1948/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1948<F: Float>(t29887: F, t7160: F, t1647: F, t1696: F, t1978: F, t25591: F, t25651: F, t25671: F, t25699: F, t27419: F, t27609: F, t27616: F, t27661: F, t29844: F, t29848: F, t29852: F, t29866: F, t29872: F, t29876: F, t29884: F, t6235: F, t6245: F, t6251: F, t6351: F, t6393: F, t7102: F, t7140: F, t7144: F, t7151: F, t7159: F, t7812: F, t7818: F, t7822: F, t7825: F, t7829: F, t7837: F) -> (F, F) {
    let t29888 = t7160 * t29887;
    let t29893 = F::cast_from(0.65854491829355115987e0_f64) * t6235 * t1978 + F::cast_from(0.13170898365871023197e1_f64) * t1647 * t7812 + F::cast_from(0.34694512752820797848e1_f64) * t25591 * t29844 - F::cast_from(0.8673628188205199462e0_f64) * t25671 * t29848 + F::cast_from(0.4336814094102599731e0_f64) * t25671 * t29852 - F::cast_from(0.8673628188205199462e0_f64) * t7825 * t7837 + F::cast_from(0.13170898365871023197e1_f64) * t7102 * t6251 + F::cast_from(0.13170898365871023197e1_f64) * t7140 * t6351 + F::cast_from(0.17347256376410398924e1_f64) * t27609 * t7829 - F::cast_from(0.65854491829355115987e0_f64) * t7140 * t6393 + F::cast_from(0.34694512752820797848e1_f64) * t7144 * t29866 - F::cast_from(0.13170898365871023197e1_f64) * t27616 * t1696 - F::cast_from(0.26020884564615598386e1_f64) * t25699 * t29872 - F::cast_from(0.8673628188205199462e0_f64) * t7144 * t29876 + F::cast_from(0.13170898365871023197e1_f64) * t25651 * t6245 - F::cast_from(0.17347256376410398924e1_f64) * t27661 * t7818 + F::cast_from(0.8673628188205199462e0_f64) * t7151 * t29884 + F::cast_from(0.17347256376410398924e1_f64) * t7159 * t29888 + F::cast_from(0.17347256376410398924e1_f64) * t27419 * t7822;
    (t29888, t29893)
}
