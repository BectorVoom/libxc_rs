//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1318/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1318<F: Float>(t30138: F, t7742: F, t30128: F, t4248: F, t1937: F, t75941: F, t114373: F, t18245: F, t7735: F, t22852: F, t28167: F, t8996: F) -> (F, F, F, F, F, F) {
    let t114434 = F::new(12.0) * t30138 * t7742;
    let t114436 = F::new(6.0) * t4248 * t30128;
    let t114438 = F::new(2.0) * t75941 * t1937;
    let t114440 = F::new(6.0) * t114373 * t1937;
    let t114442 = F::new(6.0) * t18245 * t7735;
    let t114445 = F::new(18.0) * t28167 * t8996 * t22852;
    (t114434, t114436, t114438, t114440, t114442, t114445)
}
