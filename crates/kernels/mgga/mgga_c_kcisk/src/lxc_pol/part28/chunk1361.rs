//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1361/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1361<F: Float>(t112586: F, t22267: F, t5182: F, t116509: F, t24057: F, t117108: F, t23913: F, t23918: F, t6674: F, t1790: F, t33032: F, t7242: F, t7718: F, t116465: F, t116476: F, t116479: F, t116932: F, t121258: F, t121299: F, t121314: F, t23825: F, t33002: F, t33031: F, t33059: F, t34073: F, t34137: F, t34148: F, t34192: F, t5015: F, t93749: F, t9664: F) -> (F, F, F, F, F, F) {
    let t121355 = t5182 * t112586 * t22267;
    let t121358 = t5182 * t116509 * t24057;
    let t121361 = t5182 * t117108 * t23913;
    let t121364 = t6674 * t117108 * t23918;
    let t121368 = t7242 * t33032 * t7718 * t1790;
    let t121371 = -0.8041666666666666667e-2 * t34192 * t34148 + 0.15432098765432098765e-2 * t116465 - 0.23280625000000000001e-2 * t33002 * t121258 - 0.41666666666666666668e-1 * t34073 * t34137 - t116476 - t116479 - 0.69444444444444444447e-2 * t33031 * t121314 - 0.13888888888888888889e-1 * t33031 * t116932 * t23825 - 0.20833333333333333334e-1 * t9664 * t121299 + 0.34722222222222222223e-2 * t33031 * t5015 * t33059 * t93749 - 0.22109259259259259258e-2 * t121355 - 0.7369753086419753086e-3 * t121358 - 0.44218518518518518516e-2 * t121361 + 0.3684876543209876543e-2 * t121364 + 0.34722222222222222223e-2 * t33031 * t121368;
    (t121355, t121358, t121361, t121364, t121368, t121371)
}
