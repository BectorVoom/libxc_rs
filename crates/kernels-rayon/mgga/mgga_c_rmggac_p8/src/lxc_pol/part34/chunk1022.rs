//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 1022/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk1022(t77764: f64, t118: f64, t1986: f64, t615: f64, t699: f64, t7717: f64, t75675: f64, t75681: f64, t75685: f64, t75687: f64, t69953: f64, t71552: f64, t71565: f64, t75678: f64, t77733: f64, t77737: f64, t77741: f64, t77745: f64, t77750: f64, t77755: f64, t77760: f64) -> f64 {
    let t77765 = 0.1064114997332445985e-4_f64 * t77764;
    let t77768 = t1986 * t118 * t699 * t615;
    let t77769 = t7717 * t77768;
    let t77770 = 0.53205749866622299248e-5_f64 * t77769;
    let t77772 = 0.79828278012425390427e-1_f64 * t75675;
    let t77773 = 0.1276937996798935182e-4_f64 * t75681;
    let t77774 = 0.15961724959986689775e-4_f64 * t75685;
    let t77775 = 0.1276937996798935182e-4_f64 * t75687;
    let t77776 = -t71552 - t77733 + t77737 - t77741 - t77745 - t77750 + t77755 - t77760 - t77765 + t77770 - 0.29085809927086856923e-4_f64 * t69953 + t77772 - t71565 - t75678 + t77773 + t77774 + t77775;
    t77776
}
