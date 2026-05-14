//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 972/1144 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk972<F: Float>(t28425: F, t7774: F, t1949: F, t7997: F, t8650: F, t2061: F, t7759: F, t32437: F, t32438: F, t32439: F, t32456: F, t32458: F, t32463: F, t33679: F, t33683: F, t33717: F, t33723: F, t7779: F, t8645: F, t8649: F) -> (F, F, F, F, F, F) {
    let t34044 = t28425 * t7774;
    let t34049 = t7997 * t1949;
    let t34050 = t8650 * t34049;
    let t34053 = t2061 * t7759;
    let t34054 = t8650 * t34053;
    let t34059 = -t32437 + t32438 - t32439 - 0.8673628188205199462e0 * t8645 * t7779 - 0.11423947533020470523e1 * t32463 * t34044 + t32456 - t32458 - 0.225875734067843736e-2 * t33679 - 0.56468933516960933999e-3 * t33683 + 0.57119737665102352616e0 * t8649 * t34050 + 0.57119737665102352616e0 * t8649 * t34054 + 0.7437465841810202164e-3 * t33717 + 0.14874931683620404328e-2 * t33723;
    (t34044, t34049, t34050, t34053, t34054, t34059)
}
