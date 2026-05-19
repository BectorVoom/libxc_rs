//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 997/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk997<F: Float>(t10949: F, t730: F, t2852: F, t9351: F, t10833: F, t5490: F, t5493: F, t10829: F, t713: F, t722: F, t5498: F, t1979: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t10951 = F::cast_from(0.35089341735807877242e1_f64) * t730 * t10949;
    let t10952 = t9351 * t2852;
    let t10954 = F::cast_from(0.51947577317044391277e2_f64) * t730 * t10952;
    let t10955 = t5490 * t10833;
    let t10956 = t10955 * t5493;
    let t10958 = F::cast_from(0.10254018858216406658e4_f64) * t730 * t10956;
    let t10960 = t713 * t10829 * t722;
    let t10962 = F::cast_from(0.5848223622634646207e0_f64) * t730 * t10960;
    let t10963 = t5498 * t10833;
    let t10964 = t10963 * t1979;
    (t10951, t10952, t10954, t10955, t10956, t10958, t10960, t10962, t10963, t10964)
}
