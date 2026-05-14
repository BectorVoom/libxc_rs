//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1038/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1038<F: Float>(t91794: F, t91796: F, t91799: F, t91801: F, t91804: F, t91806: F, t91809: F, t91811: F, t91814: F, t91816: F, t91818: F, t91820: F, t91822: F, t91825: F, t91828: F, t91830: F, t91832: F, t91835: F, t91837: F, t91839: F, t91841: F, t91844: F, t91847: F, t91850: F, t91852: F, t91854: F, t91857: F, t91859: F) -> (F, F) {
    let t92134 = -0.1125e1 * t91794 - 0.5625e0 * t91796 - 0.1125e1 * t91799 + 0.97125e0 * t91801 - 0.225e1 * t91804 - 0.5625e0 * t91806 + 0.1125e1 * t91809 + 0.1125e1 * t91811 + 0.809375e-1 * t91814 + 0.2428125e0 * t91816 + 0.1125e1 * t91818 - 0.485625e1 * t91820 - 0.3375e1 * t91822 - 0.485625e0 * t91825;
    let t92149 = -0.485625e0 * t91828 + 0.1875e0 * t91830 + 0.225e1 * t91832 + 0.97125e1 * t91835 + 0.2428125e0 * t91837 - 0.1875e0 * t91839 - 0.809375e-1 * t91841 - 0.97125e0 * t91844 + 0.485625e1 * t91847 + 0.485625e0 * t91850 - 0.45e1 * t91852 - 0.19425e1 * t91854 + 0.19425e1 * t91857 + 0.3375e1 * t91859;
    (t92134, t92149)
}
