//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1130/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1130<F: Float>(t173: F, t35877: F, t7470: F, t28677: F, t112268: F, t127111: F, t127651: F, t142725: F, t142867: F, t14763: F, t153017: F, t153124: F, t153193: F, t153272: F, t153276: F, t153280: F, t153285: F, t153290: F, t19072: F, t2691: F, t28666: F, t28680: F, t31465: F, t33436: F, t33928: F, t33933: F, t33934: F, t33941: F, t35879: F, t35924: F, t35929: F, t4088: F, t4113: F, t4125: F, t4126: F, t5265: F, t683: F, t7590: F, t821: F) -> (F, F) {
    let t153304 = t7470 * t173 * t35877;
    let t153305 = t28677 * t153304;
    let t153325 = F::new(0.24163653553615319118e1) * t33928 * t153193 - F::new(0.12081826776807659559e1) * t31465 * t153272 + F::new(0.21188584079044169634e-1) * t153276 * t127651 + F::new(0.30552173028732381488e-1) * t2691 * t153280 - F::new(0.15276086514366190744e-1) * t4113 * t153285 - F::new(0.36251642656102300446e0) * t112268 * t153017 + F::new(0.24167761770734866964e0) * t153290 + F::new(0.80027204934668021496e-1) * t14763 * t33933 * t35929 + F::new(0.80027204934668021496e-1) * t33934 * t33436 * t683 * t4088 - F::new(0.12004080740200203224e0) * t33941 * t33436 * t683 * t4125 - F::new(0.6041940442683716741e-1) * t153305 + F::new(0.18125821328051150223e0) * t127111 * t35879 - F::new(0.14500657062440920179e1) * t28680 * t153124 + F::new(0.48016322960800812896e0) * t142867 * t33436 * t683 * t19072 - F::new(0.24008161480400406448e0) * t142725 * t33436 * t683 * t28666 + F::new(0.13684737962323739996e1) * t5265 * t35924 * t821 - F::new(0.10263553471742804997e0) * t5265 * t7590 * t4126;
    (t153304, t153325)
}
