//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1153/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1153<F: Float>(t6305: F, t7828: F, t100705: F, t106656: F, t107532: F, t1089: F, t113669: F, t1647: F, t1668: F, t1695: F, t1976: F, t1983: F, t1984: F, t1985: F, t23820: F, t24031: F, t24048: F, t25591: F, t25611: F, t29727: F, t29751: F, t29760: F, t29809: F, t29872: F, t29875: F, t3304: F, t3318: F, t359: F, t6258: F, t7140: F, t7144: F, t7145: F, t7160: F, t7167: F, t7168: F, t7817: F, t7829: F, t93921: F, t93983: F, t93994: F, t94016: F, t94063: F, t99909: F) -> (F,) {
    let t113717 = t7828 * t6305;
    let t113728 = 0.26020884564615598386e1 * t106656 * t7829 - 0.20816707651692478709e2 * t93921 * t1985 * t107532 * t1695 - 0.39512695097613069591e1 * t7140 * t24048 + 0.52041769129231196772e1 * t7144 * t7160 * t29875 * t1695 + 0.52041769129231196772e1 * t25611 * t29727 * t1668 * t1089 - 0.78062653693846795158e1 * t100705 * t29872 + 0.52041769129231196772e1 * t25591 * t7145 * t7817 * t6258 + 0.10408353825846239354e2 * t93994 * t7145 * t1976 * t24031 + 0.19756347548806534796e1 * t1647 * t29809 - 0.4336814094102599731e0 * t7167 * t7168 * t23820 * t1089 - 0.4336814094102599731e0 * t1983 * t1984 * t359 * t113669 + 0.52041769129231196772e1 * t99909 * t29760 + 0.52041769129231196772e1 * t93983 * t113717 * t3304 - 0.26020884564615598386e1 * t94063 * t113717 * t3318 - 0.78062653693846795158e1 * t94016 * t29751 * t1668 * t1089;
    (t113728,)
}
