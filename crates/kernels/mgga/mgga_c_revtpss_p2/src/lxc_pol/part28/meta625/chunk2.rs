//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2225/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2225<F: Float>(t25577: F, t4817: F, t15711: F, t7132: F, t15655: F, t1972: F, t16060: F, t7111: F, t25539: F, t4924: F, t1028: F, t1656: F, t1665: F, t1675: F, t25495: F, t27479: F, t3220: F, t4854: F, t4887: F, t93592: F, t93691: F, t93715: F, t93722: F) -> F {
    let t100342 = F::cast_from(0.20325460441158986416e-2_f64) * t25577 * t4817;
    let t100343 = t7132 * t15711;
    let t100345 = t15655 * t1972;
    let t100359 = t7111 * t16060 / F::new(432.0);
    let t100363 = t25539 * t4924 / F::new(162.0);
    let t100364 = F::cast_from(0.96545937095505185476e-2_f64) * t93592 * t1675 - t100342 - F::cast_from(0.6351706387862183255e-4_f64) * t100343 - F::cast_from(0.85748036236139473944e-3_f64) * t100345 * t1028 - F::cast_from(0.42874018118069736972e-3_f64) * t27479 * t3220 + F::cast_from(0.45732285992607719436e-2_f64) * t93722 * t1665 + F::cast_from(0.45732285992607719436e-2_f64) * t25495 * t4854 - F::cast_from(0.14481890564325777821e-1_f64) * t93715 * t1665 - t25539 * t4887 / F::new(54.0) + t100359 + F::new(11.0) / F::new(324.0) * t93691 * t1656 - t100363;
    t100364
}
