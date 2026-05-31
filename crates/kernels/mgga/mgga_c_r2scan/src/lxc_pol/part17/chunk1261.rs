//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1261/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1261<F: Float>(t1020: F, t1133: F, t1135: F, t1137: F, t12286: F, t12288: F, t12895: F, t12897: F, t12899: F, t12901: F, t12903: F, t2410: F, t2956: F, t333: F, t3534: F, t3538: F, t3542: F, t3761: F, t3765: F, t44685: F, t839: F, t9707: F) -> F {
    let t44811 = -F::cast_from(0.3831420472412e2_f64) * t3534 * t2956 - F::cast_from(0.3831420472412e2_f64) * t1133 * t9707 + F::cast_from(0.3101306810232e2_f64) * t12286 * t1020 + F::cast_from(0.3101306810232e2_f64) * t3761 * t2410 + F::cast_from(0.1550653405116e2_f64) * t3538 * t2956 + F::cast_from(0.1550653405116e2_f64) * t1135 * t9707 - F::cast_from(0.4355305902528e1_f64) * t12288 * t1020 - F::cast_from(0.4355305902528e1_f64) * t3765 * t2410 - F::cast_from(0.2177652951264e1_f64) * t3542 * t2956 - F::cast_from(0.2177652951264e1_f64) * t1137 * t9707 - F::cast_from(0.9214113627294e1_f64) * t12895 * t839 + F::cast_from(0.367387230261e2_f64) * t12897 * t839 - F::cast_from(0.3831420472412e2_f64) * t12899 * t839 + F::cast_from(0.1550653405116e2_f64) * t12901 * t839 - F::cast_from(0.2177652951264e1_f64) * t12903 * t839 - F::cast_from(0.8704e0_f64) * t333 * t44685;
    t44811
}
