//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 708/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk708<F: Float>(t7160: F, t7828: F, t1089: F, t1668: F, t7168: F, t1984: F, t359: F, t7810: F, t1647: F, t1652: F, t1696: F, t1978: F, t1983: F, t1986: F, t342: F, t7102: F, t7140: F, t7144: F, t7151: F, t7159: F, t7167: F, t7812: F, t7818: F, t7822: F, t7825: F) -> (F, F, F, F) {
    let t7829 = t7160 * t7828;
    let t7833 = t7168 * t1668 * t1089;
    let t7837 = t1984 * t359 * t7810;
    let t7840 = F::new(0.65854491829355115987e0) * t1647 * t1978 - F::new(0.65854491829355115987e0) * t7102 * t1652 + F::new(0.65854491829355115987e0) * t342 * t7812 - F::new(0.65854491829355115987e0) * t7140 * t1696 - F::new(0.8673628188205199462e0) * t7144 * t7818 + F::new(0.8673628188205199462e0) * t7151 * t7822 - F::new(0.4336814094102599731e0) * t7825 * t1986 + F::new(0.8673628188205199462e0) * t7159 * t7829 - F::new(0.4336814094102599731e0) * t7167 * t7833 - F::new(0.4336814094102599731e0) * t1983 * t7837;
    (t7829, t7833, t7837, t7840)
}
