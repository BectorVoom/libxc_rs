//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1056/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1056<F: Float>(t79: F, t101248: F, t101507: F, t136282: F, t136313: F, t136468: F, t136469: F, t136507: F, t136572: F, t136870: F, t136903: F, t136908: F, t137007: F, t145071: F, t145075: F, t145077: F, t145120: F, t145157: F, t145195: F, t145234: F, t145266: F, t145297: F, t145339: F, t145353: F, t145361: F, t145372: F, t145409: F, t145447: F, t145474: F, t145504: F, t145531: F, t145553: F, t145580: F, t22513: F, t25643: F, t25649: F, t25653: F, t25704: F, t25722: F, t25755: F, t25794: F, t3057: F, t32133: F, t32140: F, t32146: F, t32147: F, t32228: F, t32239: F, t32241: F, t32242: F, t32259: F, t32304: F, t34430: F, t34434: F, t34440: F, t36368: F, t36390: F, t378: F, t379: F, t401: F, t428: F, t5603: F, t6450: F, t7202: F, t92278: F, t92314: F) -> F {
    let t80 = F::new(0.1e-59) < t79;
    let t145585 = piecewise3::<F>(t80, -F::cast_from(0.22227677429409423704e-2_f64) * t32228 * t145353 - F::cast_from(0.25845121844514357744e-4_f64) * t136870 * t34440 - F::cast_from(0.13784064983740990797e-3_f64) * t32304 * t145372 + F::cast_from(0.36328835404593865432e-2_f64) * t136313 * t25794 - F::cast_from(0.25537443351851851852e-1_f64) * t136572 * t6450 - F::cast_from(0.90822088511484663584e-3_f64) * t22513 * t145071 + F::cast_from(0.89080607335887169333e-3_f64) * t136908 * t34430 - F::cast_from(0.68246728907663312894e-4_f64) * t32239 * t32241 * t32242 * t3057 - F::cast_from(0.11738898233082762229e-1_f64) * t136282 * t32241 * t145077 * t379 - F::cast_from(0.20715606998445758511e-4_f64) * t101507 * t36390 * t101248 * t25755 + F::cast_from(0.70433389398496573372e-1_f64) * t136903 * t32140 * t378 * t25643 - F::cast_from(0.39525571512470170088e-4_f64) * t36368 * t137007 * t145361 * t401 + F::cast_from(0.39525571512470170088e-4_f64) * t7202 * t137007 * t145361 * t428 - F::cast_from(0.45958162518691859408e-6_f64) * t92314 * t32133 * t25649 + F::cast_from(0.22979081259345929704e-6_f64) * t92278 * t32133 * t25653 - F::cast_from(0.40859909362962962963e0_f64) * t32259 * t5603 * t34434 + F::cast_from(0.89080607335887169333e-3_f64) * t32146 * t32147 * t25722 + F::cast_from(0.51074886703703703704e-1_f64) * t136468 * t136469 * t25704 + t145580 + t145553 + t145531 + t145504 + t145474 + t145447 + t145409 + t145339 + t145297 + t145266 + t145234 + t145195 + t145157 + t145120 + F::cast_from(0.37842536879785276493e-4_f64) * t145075 - F::cast_from(0.22705522127871165896e-3_f64) * t136507, F::new(0.0));
    t145585
}
