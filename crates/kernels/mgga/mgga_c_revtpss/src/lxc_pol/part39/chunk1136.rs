//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1136/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1136<F: Float>(t14711: F, t14754: F, t14784: F, t14811: F, t14841: F, t14878: F, t14889: F, t14936: F, t136: F, t1568: F, t2457: F, t2710: F, t2470: F, t4522: F, t874: F, t10657: F, t10916: F, t10921: F, t14577: F, t14581: F, t14590: F, t14596: F, t14603: F, t14608: F, t14663: F, t1559: F, t213: F, t234: F, t2754: F, t2815: F, t4424: F, t4494: F, t4514: F, t820: F, t879: F) -> (F, F) {
    let t14939 = t14711 + t14754 + t14784 + t14811 + t14841 + t14878 + t14889 + t14936;
    let t14946 = t1568 * t136;
    let t14948 = t2710 * t14946 * t2457;
    let t14951 = t874 * t4522 * t2470;
    let t14953 = -t14577 - 0.65854491829355115987e0 * t4514 * t4494 * t2754 + 0.73171657588172351096e-2 * t14581 - 0.13170898365871023197e1 * t820 * t2815 * t4424 - t14590 + 0.23131639038696784278e-2 * t10916 + 0.54878743191129263322e-2 * t10921 + t14596 + 0.39029762157531132075e-1 * t14603 - t14608 - 0.65854491829355115987e0 * t820 * t879 * t14663 + 0.65854491829355115987e0 * t213 * t234 * t14939 - 0.65854491829355115987e0 * t820 * t10657 * t1559 + 0.11565819519348392139e-2 * t14948 - 0.13009920719177044025e-1 * t14951;
    (t14939, t14953)
}
