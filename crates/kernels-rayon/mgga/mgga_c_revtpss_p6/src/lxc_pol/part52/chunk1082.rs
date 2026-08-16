//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1082/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1082(t28425: f64, t7774: f64, t1949: f64, t7997: f64, t8650: f64, t2061: f64, t7759: f64, t32437: f64, t32438: f64, t32439: f64, t32456: f64, t32458: f64, t32463: f64, t33679: f64, t33683: f64, t33717: f64, t33723: f64, t7779: f64, t8645: f64, t8649: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t34044 = t28425 * t7774;
    let t34049 = t7997 * t1949;
    let t34050 = t8650 * t34049;
    let t34053 = t2061 * t7759;
    let t34054 = t8650 * t34053;
    let t34059 = -t32437 + t32438 - t32439 - 0.8673628188205199462e0_f64 * t8645 * t7779 - 0.11423947533020470523e1_f64 * t32463 * t34044 + t32456 - t32458 - 0.225875734067843736e-2_f64 * t33679 - 0.56468933516960933999e-3_f64 * t33683 + 0.57119737665102352616e0_f64 * t8649 * t34050 + 0.57119737665102352616e0_f64 * t8649 * t34054 + 0.7437465841810202164e-3_f64 * t33717 + 0.14874931683620404328e-2_f64 * t33723;
    (t34044, t34049, t34050, t34053, t34054, t34059)
}
