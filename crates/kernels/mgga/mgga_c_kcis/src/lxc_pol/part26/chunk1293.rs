//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1293/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1293<F: Float>(t1394: F, t8164: F, t98020: F, t28331: F, t28356: F, t5780: F, t27364: F, t29368: F, t1014: F, t29401: F, t102102: F, t102106: F, t102109: F, t102115: F, t102118: F, t8213: F, t94979: F, t99260: F, t99497: F) -> (F, F, F, F, F) {
    let t102121 = t1394 * t98020 * t8164;
    let t102124 = t5780 * t28356 * t28331;
    let t102127 = t1394 * t27364 * t29368;
    let t102129 = t1014 * t29401;
    let t102131 = -F::new(0.10306077835648148148e-4) * t94979 - F::new(0.38691203703703703703e-3) * t102102 + t99260 + F::new(0.69644166666666666664e-2) * t102106 - F::new(0.23214722222222222221e-2) * t102109 + F::new(0.92754700520833333334e-4) * t99497 * t8213 - F::new(0.18571777777777777777e-1) * t102115 + F::new(0.61905925925925925924e-2) * t102118 - F::new(0.61905925925925925925e-2) * t102121 + F::new(0.12381185185185185185e-1) * t102124 - F::new(0.23214722222222222222e-2) * t102127 + F::new(0.11349419753086419753e-1) * t102129;
    (t102121, t102124, t102127, t102129, t102131)
}
