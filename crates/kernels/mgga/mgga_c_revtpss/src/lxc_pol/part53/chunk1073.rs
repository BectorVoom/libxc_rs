//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1073/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1073<F: Float>(t7937: F, t8764: F, t2163: F, t7741: F, t651: F, t1518: F, t8756: F, t7586: F, t7742: F, t1502: F, t1911: F, t33655: F, t33659: F, t33661: F, t33664: F, t33666: F, t33669: F, t7746: F, t8761: F) -> (F, F, F) {
    let t34424 = t8764 * t7937;
    let t34428 = t2163 * t7741;
    let t34429 = t651 * t34428;
    let t34431 = t8756 * t1518;
    let t34434 = t7586 * t7742;
    let t34438 = -t1502 * t8756 + t1911 * t8761 - F::new(2.0) * t34431 * t651 - F::new(2.0) * t7586 * t7746 - t33655 + t33659 + F::new(3.0) * t33661 - t33664 - t33666 + t33669 - t34424 - F::new(2.0) * t34429 - F::new(2.0) * t34434;
    (t34428, t34431, t34438)
}
