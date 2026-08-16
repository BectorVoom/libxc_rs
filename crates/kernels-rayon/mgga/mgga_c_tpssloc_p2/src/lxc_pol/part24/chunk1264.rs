//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1264/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1264(t12303: f64, t1361: f64, t26288: f64, t12255: f64, t3788: f64, t6936: f64, t22865: f64, t6604: f64, t6937: f64, t80876: f64, t80878: f64, t80886: f64, t80889: f64, t80897: f64, t80900: f64, t80904: f64, t80906: f64, t80908: f64, t80911: f64, t80915: f64, t80918: f64, t80920: f64, t80922: f64, t80925: f64, t80928: f64, t80931: f64) -> f64 {
    let t80934 = t26288 * t1361 * t12303;
    let t80937 = t6936 * t3788 * t12255;
    let t80939 = t22865 * t6604;
    let t80940 = t80939 * t6937;
    let t80942 = -t80876 / 128.0_f64 - t80878 / 384.0_f64 - t80886 - 0.17804385437515912366e0_f64 * t80889 - 0.67826230238155856634e-1_f64 * t80897 - t80900 - t80904 / 256.0_f64 + t80906 / 256.0_f64 + 5.0_f64 / 128.0_f64 * t80908 - t80911 / 512.0_f64 - 119.0_f64 / 2304.0_f64 * t80915 - 0.60559134141210586281e-3_f64 * t80918 + 0.42391393898847410397e-2_f64 * t80920 + 0.42391393898847410397e-2_f64 * t80922 - 0.20186378047070195427e-3_f64 * t80925 - 0.20186378047070195427e-3_f64 * t80928 + 3.0_f64 / 16.0_f64 * t80931 + 0.25434836339308446237e-1_f64 * t80934 + 0.12111826828242117256e-2_f64 * t80937 - 0.33913115119077928317e-1_f64 * t80940;
    t80942
}
