//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1225/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1225<F: Float>(t12303: F, t1361: F, t26288: F, t12255: F, t3788: F, t6936: F, t22865: F, t6604: F, t6937: F, t80876: F, t80878: F, t80886: F, t80889: F, t80897: F, t80900: F, t80904: F, t80906: F, t80908: F, t80911: F, t80915: F, t80918: F, t80920: F, t80922: F, t80925: F, t80928: F, t80931: F) -> F {
    let t80934 = t26288 * t1361 * t12303;
    let t80937 = t6936 * t3788 * t12255;
    let t80939 = t22865 * t6604;
    let t80940 = t80939 * t6937;
    let t80942 = -t80876 / F::cast_from(128.0_f64) - t80878 / F::cast_from(384.0_f64) - t80886 - F::cast_from(0.17804385437515912366e0_f64) * t80889 - F::cast_from(0.67826230238155856634e-1_f64) * t80897 - t80900 - t80904 / F::cast_from(256.0_f64) + t80906 / F::cast_from(256.0_f64) + F::cast_from(5.0_f64) / F::cast_from(128.0_f64) * t80908 - t80911 / F::cast_from(512.0_f64) - F::cast_from(119.0_f64) / F::cast_from(2304.0_f64) * t80915 - F::cast_from(0.60559134141210586281e-3_f64) * t80918 + F::cast_from(0.42391393898847410397e-2_f64) * t80920 + F::cast_from(0.42391393898847410397e-2_f64) * t80922 - F::cast_from(0.20186378047070195427e-3_f64) * t80925 - F::cast_from(0.20186378047070195427e-3_f64) * t80928 + F::cast_from(3.0_f64) / F::cast_from(16.0_f64) * t80931 + F::cast_from(0.25434836339308446237e-1_f64) * t80934 + F::cast_from(0.12111826828242117256e-2_f64) * t80937 - F::cast_from(0.33913115119077928317e-1_f64) * t80940;
    t80942
}
