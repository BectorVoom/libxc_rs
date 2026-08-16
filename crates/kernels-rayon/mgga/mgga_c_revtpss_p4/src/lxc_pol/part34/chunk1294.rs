//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1294/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1294(t6305: f64, t7828: f64, t100705: f64, t106656: f64, t107532: f64, t1089: f64, t113669: f64, t1647: f64, t1668: f64, t1695: f64, t1976: f64, t1983: f64, t1984: f64, t1985: f64, t23820: f64, t24031: f64, t24048: f64, t25591: f64, t25611: f64, t29727: f64, t29751: f64, t29760: f64, t29809: f64, t29872: f64, t29875: f64, t3304: f64, t3318: f64, t359: f64, t6258: f64, t7140: f64, t7144: f64, t7145: f64, t7160: f64, t7167: f64, t7168: f64, t7817: f64, t7829: f64, t93921: f64, t93983: f64, t93994: f64, t94016: f64, t94063: f64, t99909: f64) -> f64 {
    let t113717 = t7828 * t6305;
    let t113728 = 0.26020884564615598386e1_f64 * t106656 * t7829 - 0.20816707651692478709e2_f64 * t93921 * t1985 * t107532 * t1695 - 0.39512695097613069591e1_f64 * t7140 * t24048 + 0.52041769129231196772e1_f64 * t7144 * t7160 * t29875 * t1695 + 0.52041769129231196772e1_f64 * t25611 * t29727 * t1668 * t1089 - 0.78062653693846795158e1_f64 * t100705 * t29872 + 0.52041769129231196772e1_f64 * t25591 * t7145 * t7817 * t6258 + 0.10408353825846239354e2_f64 * t93994 * t7145 * t1976 * t24031 + 0.19756347548806534796e1_f64 * t1647 * t29809 - 0.4336814094102599731e0_f64 * t7167 * t7168 * t23820 * t1089 - 0.4336814094102599731e0_f64 * t1983 * t1984 * t359 * t113669 + 0.52041769129231196772e1_f64 * t99909 * t29760 + 0.52041769129231196772e1_f64 * t93983 * t113717 * t3304 - 0.26020884564615598386e1_f64 * t94063 * t113717 * t3318 - 0.78062653693846795158e1_f64 * t94016 * t29751 * t1668 * t1089;
    t113728
}
