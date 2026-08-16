//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2420/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2420<F: Float>(t2924: F, t950: F, t14369: F, t49513: F, t13662: F, t2925: F, t959: F, t13724: F, t2940: F, t13658: F, t2907: F, t13716: F, t2929: F, t4497: F) -> (F, F, F, F, F, F) {
    let t49514 = t950 * t2924;
    let t49517 = F::cast_from(0.30762056574649219974e4_f64) * t49513 * t14369 * t49514;
    let t49520 = F::cast_from(0.35089341735807877242e1_f64) * t959 * t13662 * t2925;
    let t49522 = F::cast_from(0.31168546390226634765e3_f64) * t2940 * t13724;
    let t49525 = F::cast_from(0.10526802520742363173e2_f64) * t959 * t13658 * t2907;
    let t49529 = F::cast_from(0.51947577317044391277e2_f64) * t959 * t2929 * t13716 * t4497;
    (t49514, t49517, t49520, t49522, t49525, t49529)
}
