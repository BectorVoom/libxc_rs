# Batched compile sweep — summary

- effective jobs: `3`
- effective batch_size: `20`
- families: `lda`
- total packages swept: `43`
- pass-1 successes: `43`
- pass-2 successes: `0`
- failures: `0`
- total wall-clock: `984.9 s`
- peak-of-peak RSS: `17007.7 MB`

## Per-family

| family | ok | pass-2 | fail |
|---|---|---|---|
| lda | 43 | 0 | 0 |

## Per-batch timing + RSS

| batch_index | package_count | wall_time_s | peak_rss_mb | all_ok |
|---|---|---|---|---|
| 0 | 20 | 709.6 | 17007.7 | True |
| 1 | 20 | 272.2 | 10202.5 | True |
| 2 | 3 | 3.1 | 484.9 | True |

VERDICT: ALL_OK
