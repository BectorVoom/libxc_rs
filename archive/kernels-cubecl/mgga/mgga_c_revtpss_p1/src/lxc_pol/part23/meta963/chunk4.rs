//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3259/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3259<F: Float>(t22837: F, t9962: F, t13783: F, t3934: F, t3936: F, t46671: F, t46702: F, t46723: F, t48600: F, t48604: F, t48615: F, t5627: F, t5671: F, t6874: F, t74290: F, t74292: F, t74299: F, t74304: F, t74319: F, t74322: F, t74341: F, t74358: F, t74362: F, t85553: F, t9835: F) -> F {
    let t85839 = t9962 * t22837;
    let t85854 = -F::cast_from(0.12862205435420921092e-1_f64) * t3934 * t13783 * t6874 * t5627 - F::cast_from(0.81312004494856525161e-2_f64) * t74290 + F::cast_from(0.12004725073059526352e-1_f64) * t74292 - F::cast_from(0.22866142996303859719e-3_f64) * t74299 + F::cast_from(0.76230004213927992336e-5_f64) * t74304 - F::cast_from(0.12004725073059526352e-1_f64) * t85839 + F::cast_from(0.30011812682648815881e-2_f64) * t74319 - F::cast_from(0.8131200449485652516e-3_f64) * t74322 - F::cast_from(0.1372140075850703862e-3_f64) * t48600 + t48604 - F::cast_from(0.1829520101134271816e-3_f64) * t46671 - t48615 - F::cast_from(0.51448821741683684368e-2_f64) * t5671 * t3936 * t85553 * t9835 - F::cast_from(0.13553694749236397037e-4_f64) * t74341 + F::cast_from(0.11294745624363664198e-6_f64) * t46702 - F::cast_from(0.17006693853500995666e-1_f64) * t74358 + F::cast_from(0.37792653007779990369e-1_f64) * t46723 - F::cast_from(0.15246000842785598467e-4_f64) * t74362;
    t85854
}
