//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 929/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk929<F: Float>(t29515: F, t5513: F, t29474: F, t4491: F, t5546: F, t4474: F, t22790: F, t29469: F, t2258: F, t4462: F, t5579: F, t4454: F, t8633: F, t4458: F, t29502: F, t5540: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t29528 = t5513 * t29515;
    let t29531 = t5513 * t29474;
    let t29534 = t5546 * t4491;
    let t29540 = t5546 * t4474;
    let t29546 = t22790 * t29469;
    let t29550 = t2258 * t4462;
    let t29551 = t5579 * t29550;
    let t29554 = t8633 * t4454;
    let t29555 = t5579 * t29554;
    let t29558 = t2258 * t4458;
    let t29559 = t5579 * t29558;
    let t29562 = t5540 * t29502;
    (t29528, t29531, t29534, t29540, t29546, t29550, t29551, t29554, t29555, t29558, t29559, t29562)
}
