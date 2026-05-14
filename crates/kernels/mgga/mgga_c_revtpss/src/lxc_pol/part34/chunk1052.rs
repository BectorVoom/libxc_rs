//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1052/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1052<F: Float>(t1651: F, t7817: F, t7145: F, t25672: F, t3304: F, t6305: F, t3318: F, t7168: F, t1695: F, t7160: F, t1976: F, t6244: F, t6234: F, t6258: F, t7810: F, t1647: F, t1696: F, t1978: F, t25591: F, t25651: F, t25671: F, t25699: F, t27419: F, t27609: F, t27616: F, t27661: F, t6235: F, t6245: F, t6251: F, t6351: F, t6393: F, t7102: F, t7140: F, t7144: F, t7151: F, t7159: F, t7812: F, t7818: F, t7822: F, t7825: F, t7829: F, t7837: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t29843 = t7817 * t1651;
    let t29844 = t7145 * t29843;
    let t29848 = t25672 * t6305 * t3304;
    let t29852 = t7168 * t6305 * t3318;
    let t29865 = t7817 * t1695;
    let t29866 = t7160 * t29865;
    let t29871 = t1976 * t6244;
    let t29872 = t7145 * t29871;
    let t29875 = t1976 * t6234;
    let t29876 = t7145 * t29875;
    let t29883 = t1976 * t6258;
    let t29884 = t7145 * t29883;
    let t29887 = t7810 * t1695;
    let t29888 = t7160 * t29887;
    let t29893 = 0.65854491829355115987e0 * t6235 * t1978 + 0.13170898365871023197e1 * t1647 * t7812 + 0.34694512752820797848e1 * t25591 * t29844 - 0.8673628188205199462e0 * t25671 * t29848 + 0.4336814094102599731e0 * t25671 * t29852 - 0.8673628188205199462e0 * t7825 * t7837 + 0.13170898365871023197e1 * t7102 * t6251 + 0.13170898365871023197e1 * t7140 * t6351 + 0.17347256376410398924e1 * t27609 * t7829 - 0.65854491829355115987e0 * t7140 * t6393 + 0.34694512752820797848e1 * t7144 * t29866 - 0.13170898365871023197e1 * t27616 * t1696 - 0.26020884564615598386e1 * t25699 * t29872 - 0.8673628188205199462e0 * t7144 * t29876 + 0.13170898365871023197e1 * t25651 * t6245 - 0.17347256376410398924e1 * t27661 * t7818 + 0.8673628188205199462e0 * t7151 * t29884 + 0.17347256376410398924e1 * t7159 * t29888 + 0.17347256376410398924e1 * t27419 * t7822;
    (t29843, t29844, t29848, t29852, t29866, t29871, t29872, t29875, t29876, t29883, t29884, t29887, t29888, t29893)
}
